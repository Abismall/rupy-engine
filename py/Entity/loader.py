import os
from src.utils.logger import log_error
from src.entities.player import Player
from src.utils.files import load_file  


class EntityLoader:
    def __init__(self):
        """
        Initializes the EntityLoader.
        """
        pass

    def load_entity(self, entity_config):
        """
        Loads an entity based on the given configuration.

        :param entity_config: Dictionary containing entity data (type, attributes, and sprite paths).
        :return: A dictionary representing the entity with the properly instantiated class.
        """
        if not isinstance(entity_config, dict):
            raise TypeError(f"Error loading entity from config: '{entity_config}' is not a valid dictionary.")

        entity_config['frames'] = self.load_frames(sprite_paths=entity_config.get('sprite_paths', {}))

        entity_type = entity_config.get('type')
        attributes = entity_config.get('attributes', {})
        
        instance = self.instantiate_entity(entity_type, attributes, entity_config)

        entity_config['instance'] = instance

        return entity_config

    def instantiate_entity(self, entity_type, attributes, config):
        """
        Instantiates the correct class based on the entity type.

        :param entity_type: The type of entity (e.g., "player").
        :param attributes: Attributes to initialize the entity.
        :param config: Full entity configuration including frames.
        :return: An instantiated class object of the correct type.
        """
        if entity_type == 'player':
            return Player(keybinding_overwrites={}, entity_config=config, **attributes)
        else:
            log_error(f"Unknown entity type: {entity_type}. Returning None.")
            return None

    def load_frames(self, sprite_paths):
        """
        Loads and scales frames for each action from the specified directories.

        :param sprite_paths: Dictionary of action names to sprite folder paths.
        :return: A dictionary with action names as keys and lists of frames as values.
        """
        frames = {}
        for action, path in sprite_paths.items():
            try:
                frames[action] = [load_file(os.path.join(path, file)) for file in os.listdir(path)]
                if not frames[action]:
                    raise FileNotFoundError(f"No valid image files found in {path} for action '{action}'")
            except Exception as e:
                log_error(f"Error loading frames for action '{action}' from path '{path}': {e}")
                frames[action] = []  
        return frames

    def load_entities_from_config(self, config_file):
        """
        Loads multiple entities from a configuration file.

        :param config_file: Path to a JSON file containing entity configurations.
        :return: A list of entities loaded from the configuration file.
        """
        import json
        entities = []
        with open(config_file, 'r') as file:
            for entity in json.load(file).get('entities', []):
                entities.append(self.load_entity(entity))
        return entities