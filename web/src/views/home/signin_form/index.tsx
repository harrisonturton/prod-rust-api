import { useState } from "react";
import { TextField } from "base/form";
import { Button } from "base/button";
import { Router, getEditorRoute } from "base/router";
import styles from "./styles.module.scss";

export const UsernamePasswordForm = () => {
    let router = new Router();

    let [email, setEmail] = useState<string>("");
    let [emailError, setEmailError] = useState<string | null>(null);

    let [password, setPassword] = useState<string>("");
    let [passwordError, setPasswordError] = useState<string | null>(null);

    let [loading, setLoading] = useState<boolean>(false);

    const onSubmit = async () => {
        setEmailError(null);
        setPasswordError(null);
        let validated = true;
        if (email.trim().length == 0) {
            setEmailError("Please fill in the email field");
            validated = false;
        }
        if (password.trim().length == 0) {
            setPasswordError("Please fill in the password field");
            validated = false;
        }
        if (!validated) {
            return;
        }
        setLoading(true);
        await new Promise((resolve) => setTimeout(resolve, 1000));
        setLoading(false);
        router.pushRoute(getEditorRoute());
        console.log(getEditorRoute());
    };

    return (
        <form className={styles.formContainer}>
            <span className={styles.signInLabel}>Sign in to your account</span>
            <TextField
                required={true}
                valid={emailError == null}
                type="text"
                placeholder="Email"
                value={email}
                onChange={setEmail}
                autoFocus={true}
            />
            {emailError && (
                <span className={styles.formError}>{emailError}</span>
            )}
            <TextField
                required={true}
                valid={passwordError == null}
                type="password"
                placeholder="Password"
                value={password}
                onChange={setPassword}
            />
            {passwordError && (
                <span className={styles.formError}>{passwordError}</span>
            )}
            <Button label="Sign In" onClick={onSubmit} loading={loading} />
        </form>
    );
};
