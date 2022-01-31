// Dirty, dirty hack. Won't work for values that lose type information when
// serialised to JSON, like dates, regexes, undefined values, etc.
// TODO(harry): Replace with a more robust deep clone implementation
export const deepClone = <T>(obj: T): T => JSON.parse(JSON.stringify(obj));
