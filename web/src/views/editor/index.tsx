import type { NextPage } from "next";
import Head from "next/head";
import TabNav from "views/editor/app-nav";
import Toolbar from "views/editor/toolbar";
import FileTabs from "views/editor/file-tabs";
import StatusBar from "views/editor/status-bar";
import createFileTree from "views/editor/file-nav";
import styles from "./styles.module.scss";

import { HttpClient } from "services/http";
import { UserClient, User } from "services/user";
import { useEffect, useState } from "react";

const Editor: NextPage = () => {
    const { FileTree } = createFileTree();

    const [ users, setUsers ] = useState<User[]>([]);

    const loadUsers = async () => {
        let httpClient = new HttpClient("http://localhost:8000");
        let userClient = new UserClient(httpClient);
        let userRes = await userClient.listUsers();
        setUsers(userRes.users);
    };

    useEffect(() => {
        loadUsers();
    }, []);

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
                        <div>
                            {users.map(user => (
                                <div>
                                    <span>{user.id}</span>
                                    <span>{user.email}</span>
                                    <span>{user.created_at}</span>
                                </div>
                            ))}
                        </div>
                    </div>
                </div>
                <StatusBar />
            </main>
        </div>
    );
};

export default Editor;
