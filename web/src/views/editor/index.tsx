import type { NextPage } from "next";
import Head from "next/head";
import TabNav from "views/editor/app-nav";
import Toolbar from "views/editor/toolbar";
import FileTabs from "views/editor/file-tabs";
import StatusBar from "views/editor/status-bar";
import createFileTree from "views/editor/file-nav";
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
