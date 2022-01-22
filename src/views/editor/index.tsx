import type { NextPage } from "next";
import Head from "next/head";
import FileTree from "components/file-tree";
import styles from "./styles.module.css";

const Editor: NextPage = () => (
    <div>
        <Head>
            <title>Editor Page</title>
            <meta name="description" content="Editor page"/>
        </Head>
        <main className={styles.content}>
            <p>Editor</p>
        </main>
    </div>
);

export default Editor;