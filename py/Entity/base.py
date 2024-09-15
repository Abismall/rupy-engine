
# from src.graphics.render import RenderCommand


# class Entity:
#     def __init__(self, frames=None, **kwargs):
#         """
#         Initializes the base Entity with shared attributes and frames provided externally.

#         :param frames: Optional dictionary of frames loaded externally.
#         :param kwargs: Keyword arguments for initializing entity attributes.
#         """

#         self.x = kwargs.get('x', 0)
#         self.y = kwargs.get('y', 0)
#         self.vx = kwargs.get('vx', 0)
#         self.vy = kwargs.get('vy', 0)
#         self.velocity = kwargs.get('velocity', 1)
#         self.ground_level = kwargs.get('ground_level', 0)
#         self.is_facing = kwargs.get('facing_dir', 'Right')
#         self.current_action = kwargs.get('current_action', 'idle')
#         self.is_jumping = kwargs.get('is_jumping', False)
#         self.frames = frames if frames is not None else {}
#         self.render_queue = []

#     def update(self):
#         """
#         Update the entity state. Meant to be overridden by subclasses.
#         """
#         pass

#     def items(self):
#         """
#         Returns the render queue and clears it to prepare for the next frame.
#         """
#         queue = self.render_queue[:]
#         self.render_queue.clear()
#         return queue

#     def set_render_command(self, action_name, frame=None, update_mode='update'):
#         """
#         Sets the render command based on the current action.

#         :param action_name: The name of the action to set.
#         :param frame: The frame or image to render.
#         :param update_mode: The rendering mode ('flip' or 'update').
#         """
#         self.render_queue.append(RenderCommand(
#             entity=self,
#             command_type=action_name,
#             flip_x=self.is_facing == "Left",
#             start_pos=(self.x, self.y),
#             image=frame,
#             update_mode=update_mode
#         ))
