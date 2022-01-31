import type { NextPage } from "next";
import Head from "next/head";
import TabNav from "base/app-nav";
import Toolbar from "base/toolbar";
import FileTabs from "base/file-tabs";
import StatusBar from "base/status-bar";
import createFileTree from "base/file-nav";
import styles from "./styles.module.scss";

const Editor: NextPage = () => {
    const { FileTree } = createFileTree();
    return (
        <div>
            <Head>
                <title>Editor Page</title>
                <meta name="description" content="Editor page" />
            </Head>
            <main className={styles.root}>
                <Toolbar />
                <div className={styles.mainContent}>
                    <TabNav />
                    <FileTree />
                    <div className={styles.fileContainer}>
                        <FileTabs />
                    </div>
                </div>
                <StatusBar />
            </main>
        </div>
    );
};

export default Editor;
