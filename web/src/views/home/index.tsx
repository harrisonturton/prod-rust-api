import type { NextPage } from "next";
import Head from "next/head";
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
        </main>
    </div>
);

export default Home;
