const Ajv = require("ajv");
const addFormats = require("ajv-formats");

const authSchema = require("./auth_schema.json");
const userSchema = require("../user/user_schema.json");

let ajv = new Ajv();
addFormats(ajv, ["date", "time", "date-time"]);
//ajv.addSchema(authSchema);
//ajv.addSchema(userSchema);

// https://stackoverflow.com/questions/63905928/validate-using-a-specific-definition-with-ajv
const body = {
	email: "testing",
	password: "bad",
};

var v = ajv.validate(authSchema.definitions.SignInResponse, body);
console.log("SignInResponse:", v);
console.log("SignInResponse:", v.errors);

const user = {
	id: "user_id",
	email: "email",
	created_at: "1970-01-01T10:05:08",
}

var v = ajv.validate(userSchema.definitions.User, user);
console.log("User:", v);
console.log("User:", v.errors);

var validate = ajv.compile(userSchema.definitions.User);
console.log("Compiling sub schemas:", validate(user));

console.log(userSchema.definitions.User);
