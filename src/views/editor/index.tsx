import type { NextPage } from "next";
import Head from "next/head";
import TabNav from "base/tab-nav";
import Toolbar from "base/toolbar";
import createFileTree from "base/file-nav";
import styles from "./styles.module.scss";

const Editor: NextPage = () => {
    const { FileTree } = createFileTree();
    return (
        <div>
            <Head>
                <title>Editor Page</title>
                <meta name="description" content="Editor page"/>
            </Head>
            <main className={styles.root}>
                <Toolbar/>
                <section className={styles.mainContent}>
                    <TabNav/>
                    <FileTree/>
                </section>
            </main>
        </div>
    );
};

export default Editor;