import { checkSchema } from "../schema";
import { FakeSchema } from "./schema_fake_schema";
import fakeSchema from "./schema_fake_schema.json";

describe("checkSchema", () => {
    it("throws on values that don't match schema", () => {
        let value = { badParam: "bad" };
        let schema = fakeSchema.definitions.FakeSchema;
        expect(() => checkSchema(value, schema)).toThrow();
    });

    it("does not throw on values that match schema", () => {
        let value: FakeSchema = {
            numberParam: 1,
            stringParam: "string",
            nestedObjectParam: {
                booleanParam: true,
            },
        };
        let schema = fakeSchema.definitions.FakeSchema;
        checkSchema(value, schema);
    });
});
