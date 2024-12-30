import { PublicKey } from '@solana/web3.js';


type Entity = string; 
type Region = string; 

export interface World {
  discriminator: Uint8Array; 
  name: string; 
  description: string; 
  entities: Entity[]; 
  regions: Region[]; 
  instances: Map<Region, Map<Entity, bigint>>; 
  isLaunched: boolean; 
  worldAuthority: PublicKey; 
  bump: number; 
}

export class WorldUtil {
  static SPL_DISCRIMINATOR_SLICE: Uint8Array = new Uint8Array([/* Insert discriminator slice bytes */]);
  static UNINITIALIZED_DISCRIMINATOR: Uint8Array = new Uint8Array([/* Insert uninitialized bytes */]);

  /**
   * Checks if the World is initialized.
   * @param world - The world object to check.
   */
  static isInitialized(world: World): boolean {
    return WorldUtil.arrayEquals(world.discriminator, WorldUtil.SPL_DISCRIMINATOR_SLICE);
  }

  /**
   * Checks if the World is uninitialized.
   * @param world - The world object to check.
   */
  static isUninitialized(world: World): boolean {
    return WorldUtil.arrayEquals(world.discriminator, WorldUtil.UNINITIALIZED_DISCRIMINATOR);
  }

  static create(
    name: string,
    description: string,
    worldAuthority: PublicKey,
    regions: Region[],
    entities: Entity[],
    bump: number,
    preload: boolean
  ): World {
    const instances = new Map<Region, Map<Entity, bigint>>();

    if (preload) {
      regions.forEach((region) => {
        const regionMap = new Map<Entity, bigint>();
        entities.forEach((entity) => {
          regionMap.set(entity, BigInt(0)); // u64::MIN in Rust is 0
        });
        instances.set(region, regionMap);
      });
    }

    return {
      discriminator: WorldUtil.SPL_DISCRIMINATOR_SLICE,
      name,
      description,
      entities,
      regions,
      instances,
      isLaunched: false,
      worldAuthority,
      bump,
    };
  }

  /**
   * Helper function to compare two Uint8Arrays.
   * @param a - First array to compare.
   * @param b - Second array to compare.
   */
  private static arrayEquals(a: Uint8Array, b: Uint8Array): boolean {
    if (a.length !== b.length) return false;
    return a.every((value, index) => value === b[index]);
  }
}
