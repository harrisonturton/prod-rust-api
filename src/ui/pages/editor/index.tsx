import type { NextPage } from "next";
import Head from "next/head";
import styles from "./styles.module.css";

const Editor: NextPage = () => (
    <div>
        <Head>
            <title>Editor Page</title>
            <meta name="description" content="Editor page"/>
        </Head>
        <main className={styles.content}>
            <h1>Editor</h1>
        </main>
    </div>
);

export default Editor;