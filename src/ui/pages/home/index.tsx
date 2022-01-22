import type { NextPage } from "next";
import Head from "next/head";
import styles from "./styles.module.css";

const Home: NextPage = () => (
    <div>
        <Head>
            <title>Home Page</title>
            <meta name="description" content="Home page"/>
        </Head>
        <main className={styles.content}>
            <h1>Home</h1>
        </main>
    </div>
);

export default Home;