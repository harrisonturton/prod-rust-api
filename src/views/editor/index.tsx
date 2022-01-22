import type { NextPage } from "next";
import Head from "next/head";
import FileTree from "base/file-tree";
import styles from "./styles.module.scss";

const Editor: NextPage = () => (
    <div>
        <Head>
            <title>Editor Page</title>
            <meta name="description" content="Editor page"/>
        </Head>
        <main className={styles.content}>
            <FileTree/>
        </main>
    </div>
);

export default Editor;