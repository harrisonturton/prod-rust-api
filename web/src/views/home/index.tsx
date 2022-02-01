import type { NextPage } from "next";
import Head from "next/head";
import Link from "next/link";
import { getEditorRoute } from "base/routes";
import { UsernamePasswordForm } from "./signin_form";
import styles from "./styles.module.scss";

const Home: NextPage = () => (
    <div>
        <Head>
            <title>Home Page</title>
            <meta name="description" content="Home page" />
        </Head>
        <main className={styles.content}>
            <UsernamePasswordForm />
            <Link href={getEditorRoute()}>
                <a className={styles.homeLabel}>Don't have an account?</a>
            </Link>
        </main>
    </div>
);

export default Home;
