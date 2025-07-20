class _Wrapper<T> {
  public constructor(plain: T) {
    Object.assign(this, plain);
  }
}

type Wrapper<T> = new (plain: T) => T & _Wrapper<T>;
export function Wrapper<T>(): Wrapper<T> {
  // deno-lint-ignore no-explicit-any
  return _Wrapper<T> as any;
}
