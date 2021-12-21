/* eslint-disable */

export class ExternalObject<T> {
  readonly '': {
    readonly '': unique symbol
    [K: symbol]: T
  }
}
export function portsList(): Record<string, object>
export class Port {
  readonly path: string
  /** This is the constructor */
  constructor(path: string)
  get usedPath(): string
  write(data: string): void
  read(): string
}
