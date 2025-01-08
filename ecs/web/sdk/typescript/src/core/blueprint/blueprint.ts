
export type Component = string;

// Enum for different types of component values.
export enum ComponentValue {
  String = "String",
  Integer = "Integer",
  Float = "Float",
  Boolean = "Boolean",
}


export class ComponentValueWrapper {
  private value: string | number | boolean;

  constructor(value: string | number | boolean) {
    this.value = value;
  }

  unwrapString(): string {
    return this.value as string;
  }

  unwrapFloat(): number {
    return this.value as number;
  }

  unwrapInt(): number {
    return this.value as number;
  }

  unwrapBool(): boolean {
    return this.value as boolean;
  }

  equals(other: ComponentValueWrapper): boolean {
    return this.value === other.value;
  }
}
