import type { NextPage } from "next";
import Head from "next/head";
import Link from "next/link";
import { getEditorRoute } from "base/routes";
import { useState } from "react";
import { TextField, Button } from "base/input";
import styles from "./styles.module.scss";

const Login = () => {
    let [ username, setEmail ] = useState<string>("");
    let [ password, setPassword ] = useState<string>("");
    let [ loading, setLoading ] = useState<boolean>(false);
    let [ error, setError ] = useState<string[]>([]);

    const onSubmit = () => {
        console.log("Submitted!");
    };

    return (
        <div className={styles.formContainer}>
            <span className={styles.signInLabel}>Sign in to your account</span>
            <TextField
                className={styles.inputField}
                type="text"
                placeholder="Email"
                value={username}
                onChange={setEmail}
            />
            <TextField
                className={styles.inputField}
                type="password"
                placeholder="Password"
                value={password}
                onChange={setPassword}
            />
            <Button
                label="Sign In"
                onClick={onSubmit}
            />
            {loading && <span>Loading</span>}
        </div>
    );
};

const Home: NextPage = () => (
    <div>
        <Head>
            <title>Home Page</title>
            <meta name="description" content="Home page"/>
        </Head>
        <main className={styles.content}>
            <Login/>
            <Link href={getEditorRoute()}>
                <a className={styles.homeLabel}>Don't have an account?</a>
            </Link>
        </main>
    </div>
);

export default Home;