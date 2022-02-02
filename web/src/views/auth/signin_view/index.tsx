import type { NextPage } from "next";
import Head from "next/head";
import { EmailPasswordForm } from "../email_password_form";
import styles from "./styles.module.scss";

const SignInView: NextPage = () => (
    <div>
        <Head>
            <title>Sign In</title>
            <meta name="description" content="Sign In" />
        </Head>
        <main className={styles.content}>
            <EmailPasswordForm />
        </main>
    </div>
);

export default SignInView;
