export type DiscriminateUnion<T, K extends keyof T, V extends T[K]> =
  T extends Record<K, V> ? T : never;

type MapDiscriminatedUnion<
  T extends Record<K, string>,
  K extends keyof T,
  TRes,
> = {
  [V in T[K]]: (val: DiscriminateUnion<T, K, V>) => TRes;
};

export function mapDiscriminatedUnion<
  TUnion extends Record<TKey, string>,
  TKey extends keyof TUnion,
  TMappers extends MapDiscriminatedUnion<TUnion, TKey, unknown>,
  TRes = TMappers extends MapDiscriminatedUnion<TUnion, TKey, infer TRes>
    ? TRes
    : never,
>(data: TUnion, key: TKey, mappers: TMappers): TRes {
  const fn = mappers[data[key]];

  if (!fn) {
    return null as TRes;
  }

  // @ts-expect-error typescript prevents this check because it is discriminated union but we just unwrapped the correct fn for the type
  return fn(data) as TRes;
}
