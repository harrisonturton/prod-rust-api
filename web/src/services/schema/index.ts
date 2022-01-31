import Ajv from "ajv";
import { Schema, ValidateFunction } from "ajv/dist/types";

/**
 * This is exported so other modules can use the schema type without touching
 * `ajv`.
 */
export type { Schema } from "ajv/dist/types";

/**
 * Ajv is most efficient when all the schemas are loaded up-front. This is the
 * "global instance cache" where every service schema is compiled and imported.
 * [Docs.](https://ajv.js.org/guide/managing-schemas.html#using-ajv-instance-cache)
 */
export const ajv = new Ajv();

/**
 * Compiles a JSON schema validator function and caches it for future use. This
 * means it may be a bit slower the first time it is called for a given schema,
 * but will be much faster afterwards.
 *
 * As an example, to get the validator function for an interface `MyRequest`, we
 * need to pass a reference to it's schema directly:
 *
 * ```
 * import myServiceSchema from "services/my_service/my_service_schema.json";
 * import { getValidator } from "base/services/schema";
 *
 * const validate = getValidator(myServiceSchema.definitions.MyRequest);
 * if (validate(some_object)) {
 *   console.log("yay!");
 * }
 * ```
 *
 * @param schema the definitions object of the schema you want to enforce.
 * @returns a function to validate an object against the schema.
 */
export function getValidator(schema: Schema): ValidateFunction {
    return ajv.compile(schema);
}

/**
 * Throw if the value is invalid.
 *
 * @param schema the schema you want to test the value against.
 * @param value the value you want to validate.
 */
export function rejectInvalidSchema(
    value: object,
    schema: Schema,
    message?: string
) {
    let validate = getValidator(schema);
    if (!validate(value)) {
        throw Error(message ?? "value rejected by schema");
    }
}
