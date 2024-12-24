import { MemoryAdapter } from './memory';

describe('MemoryAdapter', () => {
    let adapter: MemoryAdapter;

    beforeEach(() => {
        adapter = new MemoryAdapter('testBlueprint', 'testProgramId', 'http://localhost');
    });

    test('migrate should initialize instances if empty', async () => {
        await adapter.migrate();
        const instances = adapter['instances']; // Accessing private property for testing
        expect(instances.size).toBe(1);
        expect(instances.has('testBlueprint')).toBe(true);
    });

    test('create should create a new instance', async () => {
        await adapter.create();
        const instances = adapter['instances']; // Accessing private property for testing
        expect(instances.size).toBe(1);
        expect(instances.get('testBlueprint')).toEqual({
            createdAt: expect.any(Date),
            data: {},
        });
    });

    test('create should not create a duplicate instance', async () => {
        await adapter.create(); // First creation
        const initialSize = adapter['instances'].size;
        await adapter.create(); // Attempt to create again
        expect(adapter['instances'].size).toBe(initialSize); // Size should remain the same
    });

    test('get should retrieve data for a given entity ID', async () => {
        await adapter.set('entity1', { name: 'Test Entity' });
        const data = await adapter.get('entity1');
        expect(data).toEqual({ name: 'Test Entity' });
    });

    test('get should throw an error if entity does not exist', async () => {
        await expect(adapter.get('nonExistentEntity')).rejects.toThrow('Entity with ID nonExistentEntity does not exist.');
    });

    test('set should store data for a given entity ID', async () => {
        await adapter.set('entity2', { name: 'Another Entity' });
        const data = await adapter.get('entity2');
        expect(data).toEqual({ name: 'Another Entity' });
    });

    test('delete should remove data for a given entity ID', async () => {
        await adapter.set('entity3', { name: 'Entity to Delete' });
        await adapter.delete('entity3');
        await expect(adapter.get('entity3')).rejects.toThrow('Entity with ID entity3 does not exist.');
    });

    test('delete should throw an error if entity does not exist', async () => {
        await expect(adapter.delete('nonExistentEntity')).rejects.toThrow('Entity with ID nonExistentEntity does not exist.');
    });
});
