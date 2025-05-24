"""
Minimal Python bridge for DaVinci Resolve API access.
This module provides a clean interface between Rust and the DaVinci Resolve Python API.
"""

import json
import traceback
from typing import Any, Dict, Optional, List, Union

try:
    import DaVinciResolveScript as dvr_script
    RESOLVE_AVAILABLE = True
except ImportError:
    RESOLVE_AVAILABLE = False
    dvr_script = None


class ResolveBridge:
    """Bridge class for accessing DaVinci Resolve API from Rust."""
    
    def __init__(self):
        """Initialize the bridge and connect to DaVinci Resolve."""
        self.resolve = None
        self.project_manager = None
        self.project = None
        self.media_pool = None
        self.timeline = None
        
        if RESOLVE_AVAILABLE:
            try:
                self.resolve = dvr_script.scriptapp("Resolve")
                if self.resolve:
                    self.project_manager = self.resolve.GetProjectManager()
                    self.project = self.project_manager.GetCurrentProject()
                    if self.project:
                        self.media_pool = self.project.GetMediaPool()
                        self.timeline = self.project.GetCurrentTimeline()
            except Exception as e:
                print(f"Warning: Could not connect to DaVinci Resolve: {e}")
    
    def call_api(self, method: str, args: Dict[str, Any]) -> Dict[str, Any]:
        """
        Generic API call handler that routes to appropriate DaVinci Resolve methods.
        
        Args:
            method: The API method to call
            args: Arguments for the method
            
        Returns:
            Dict containing the result or error information
        """
        try:
            if not RESOLVE_AVAILABLE:
                return {"error": "DaVinci Resolve Python API not available"}
            
            if not self.resolve:
                return {"error": "DaVinci Resolve is not running"}
            
            # Refresh current objects
            self._refresh_current_objects()
            
            # Route to appropriate handler
            if method.startswith("project_"):
                return self._handle_project_method(method, args)
            elif method.startswith("timeline_"):
                return self._handle_timeline_method(method, args)
            elif method.startswith("media_"):
                return self._handle_media_method(method, args)
            elif method.startswith("color_"):
                return self._handle_color_method(method, args)
            elif method.startswith("render_"):
                return self._handle_render_method(method, args)
            else:
                return self._handle_general_method(method, args)
                
        except Exception as e:
            return {"error": f"API call failed: {str(e)}", "traceback": traceback.format_exc()}
    
    def _refresh_current_objects(self):
        """Refresh current project, timeline, etc."""
        try:
            if self.project_manager:
                self.project = self.project_manager.GetCurrentProject()
                if self.project:
                    self.media_pool = self.project.GetMediaPool()
                    self.timeline = self.project.GetCurrentTimeline()
        except Exception:
            pass
    
    def _handle_general_method(self, method: str, args: Dict[str, Any]) -> Dict[str, Any]:
        """Handle general API methods."""
        if method == "is_running":
            return {"result": self.resolve is not None}
        
        elif method == "get_current_project_name":
            if self.project:
                name = self.project.GetName()
                return {"result": name}
            return {"result": None}
        
        elif method == "get_timelines":
            if self.project:
                timeline_count = self.project.GetTimelineCount()
                timelines = []
                for i in range(1, timeline_count + 1):
                    timeline = self.project.GetTimelineByIndex(i)
                    if timeline:
                        timelines.append(timeline.GetName())
                return {"result": timelines}
            return {"result": []}
        
        elif method == "switch_page":
            page = args.get("page", "")
            success = self.resolve.OpenPage(page)
            return {"result": success}
        
        else:
            return {"error": f"Unknown general method: {method}"}
    
    def _handle_project_method(self, method: str, args: Dict[str, Any]) -> Dict[str, Any]:
        """Handle project-related API methods."""
        if method == "project_create":
            name = args.get("name", "")
            if self.project_manager:
                project = self.project_manager.CreateProject(name)
                if project:
                    self.project = project
                    self.media_pool = project.GetMediaPool()
                    return {"result": f"Created project '{name}'"}
            return {"error": f"Failed to create project '{name}'"}
        
        elif method == "project_open":
            name = args.get("name", "")
            if self.project_manager:
                project = self.project_manager.LoadProject(name)
                if project:
                    self.project = project
                    self.media_pool = project.GetMediaPool()
                    return {"result": f"Opened project '{name}'"}
            return {"error": f"Failed to open project '{name}'"}
        
        elif method == "project_save":
            if self.project_manager:
                success = self.project_manager.SaveProject()
                return {"result": success}
            return {"error": "No project manager available"}
        
        elif method == "project_close":
            if self.project_manager:
                success = self.project_manager.CloseProject(self.project)
                if success:
                    self.project = None
                    self.media_pool = None
                    self.timeline = None
                return {"result": success}
            return {"error": "No project manager available"}
        
        else:
            return {"error": f"Unknown project method: {method}"}
    
    def _handle_timeline_method(self, method: str, args: Dict[str, Any]) -> Dict[str, Any]:
        """Handle timeline-related API methods."""
        if method == "timeline_create":
            name = args.get("name", "")
            if self.media_pool:
                timeline = self.media_pool.CreateEmptyTimeline(name)
                if timeline:
                    return {"result": f"Created timeline '{name}'"}
            return {"error": f"Failed to create timeline '{name}'"}
        
        elif method == "timeline_delete":
            name = args.get("name", "")
            if self.project:
                # Find timeline by name
                timeline_count = self.project.GetTimelineCount()
                for i in range(1, timeline_count + 1):
                    timeline = self.project.GetTimelineByIndex(i)
                    if timeline and timeline.GetName() == name:
                        success = self.project.DeleteTimeline(timeline)
                        return {"result": success}
            return {"error": f"Timeline '{name}' not found"}
        
        elif method == "timeline_set_current":
            name = args.get("name", "")
            if self.project:
                # Find timeline by name
                timeline_count = self.project.GetTimelineCount()
                for i in range(1, timeline_count + 1):
                    timeline = self.project.GetTimelineByIndex(i)
                    if timeline and timeline.GetName() == name:
                        success = self.project.SetCurrentTimeline(timeline)
                        if success:
                            self.timeline = timeline
                        return {"result": success}
            return {"error": f"Timeline '{name}' not found"}
        
        else:
            return {"error": f"Unknown timeline method: {method}"}
    
    def _handle_media_method(self, method: str, args: Dict[str, Any]) -> Dict[str, Any]:
        """Handle media pool related API methods."""
        if method == "media_import":
            file_path = args.get("file_path", "")
            if self.media_pool:
                clips = self.media_pool.ImportMedia([file_path])
                if clips:
                    return {"result": f"Imported media: {file_path}"}
            return {"error": f"Failed to import media: {file_path}"}
        
        elif method == "media_create_bin":
            name = args.get("name", "")
            if self.media_pool:
                bin_item = self.media_pool.CreateBin(name)
                if bin_item:
                    return {"result": f"Created bin '{name}'"}
            return {"error": f"Failed to create bin '{name}'"}
        
        else:
            return {"error": f"Unknown media method: {method}"}
    
    def _handle_color_method(self, method: str, args: Dict[str, Any]) -> Dict[str, Any]:
        """Handle color grading related API methods."""
        # Placeholder for color grading methods
        return {"error": f"Color method not implemented: {method}"}
    
    def _handle_render_method(self, method: str, args: Dict[str, Any]) -> Dict[str, Any]:
        """Handle rendering related API methods."""
        # Placeholder for rendering methods
        return {"error": f"Render method not implemented: {method}"} 