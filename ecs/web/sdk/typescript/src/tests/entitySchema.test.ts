import { Entity, Instance, isEntity, isInstance } from '../core/entity/entitySchema';

describe('Entity Schema', () => {
  test('validates correct Entity object', () => {
    const validEntity: Entity = {
      id: 'entity-1',
      type: 'player',
      components: { health: 100 },
      createdAt: new Date(),
      updatedAt: new Date()
    };
    
    expect(isEntity(validEntity)).toBe(true);
  });

  test('rejects invalid Entity object', () => {
    const invalidEntity = {
      id: 'entity-1',
      type: 'player',
      components: { health: 100 }
      // Missing createdAt and updatedAt
    };
    
    expect(isEntity(invalidEntity)).toBe(false);
  });

  test('rejects Entity with invalid id type', () => {
    const invalidEntity = {
      id: 123, // Should be string
      type: 'player',
      components: { health: 100 },
      createdAt: new Date(),
      updatedAt: new Date()
    };
    
    expect(isEntity(invalidEntity)).toBe(false);
  });

  test('rejects Entity with empty components', () => {
    const invalidEntity = {
      id: 'entity-1',
      type: 'player',
      components: {},
      createdAt: new Date(),
      updatedAt: new Date()
    };
    
    expect(isEntity(invalidEntity)).toBe(false);
  });

  test('rejects Entity with invalid createdAt format', () => {
    const invalidEntity = {
      id: 'entity-1',
      type: 'player',
      components: { health: 100 },
      createdAt: '2023-01-01', // Should be Date object
      updatedAt: new Date()
    };
    
    expect(isEntity(invalidEntity)).toBe(false);
  });
});

describe('Instance Schema', () => {
  test('validates correct Instance object', () => {
    const validInstance: Instance = {
      entityId: 'entity-1',
      worldId: 'world-1',
      state: { position: { x: 0, y: 0 } },
      createdAt: new Date(),
      updatedAt: new Date()
    };
    
    expect(isInstance(validInstance)).toBe(true);
  });

  test('rejects invalid Instance object', () => {
    const invalidInstance = {
      entityId: 'entity-1',
      worldId: 'world-1',
      state: { position: { x: 0, y: 0 } }
      // Missing createdAt and updatedAt
    };
    
    expect(isInstance(invalidInstance)).toBe(false);
  });

  test('rejects Instance with invalid state structure', () => {
    const invalidInstance = {
      entityId: 'entity-1',
      worldId: 'world-1',
      state: 'invalid-state', // Should be object
      createdAt: new Date(),
      updatedAt: new Date()
    };
    
    expect(isInstance(invalidInstance)).toBe(false);
  });

  test('rejects Instance with missing worldId', () => {
    const invalidInstance = {
      entityId: 'entity-1',
      state: { position: { x: 0, y: 0 } },
      createdAt: new Date(),
      updatedAt: new Date()
    };
    
    expect(isInstance(invalidInstance)).toBe(false);
  });

  test('rejects Instance with invalid updatedAt format', () => {
    const invalidInstance = {
      entityId: 'entity-1',
      worldId: 'world-1',
      state: { position: { x: 0, y: 0 } },
      createdAt: new Date(),
      updatedAt: '2023-01-01' // Should be Date object
    };
    
    expect(isInstance(invalidInstance)).toBe(false);
  });
});
