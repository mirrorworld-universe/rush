export interface Entity {
  id: string;
  type: string;
  components: Record<string, any>;
  createdAt: Date;
  updatedAt: Date;
}

export interface Instance {
  entityId: string;
  worldId: string;
  state: Record<string, any>;
  createdAt: Date;
  updatedAt: Date;
}

export function isEntity(obj: any): obj is Entity {
  return obj &&
    typeof obj.id === 'string' &&
    typeof obj.type === 'string' &&
    typeof obj.components === 'object' &&
    Object.keys(obj.components).length > 0 && // Reject empty components
    obj.createdAt instanceof Date &&
    obj.updatedAt instanceof Date;
}

export function isInstance(obj: any): obj is Instance {
  return obj &&
    typeof obj.entityId === 'string' &&
    typeof obj.worldId === 'string' &&
    typeof obj.state === 'object' &&
    obj.createdAt instanceof Date &&
    obj.updatedAt instanceof Date;
}
