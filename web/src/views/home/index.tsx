import type { NextPage } from "next";
import Head from "next/head";
import Link from "next/link";
import { getEditorRoute } from "base/routes";
import styles from "./styles.module.scss";

const Home: NextPage = () => (
    <div>
        <Head>
            <title>Home Page</title>
            <meta name="description" content="Home page"/>
        </Head>
        <main className={styles.content}>
            <span className={styles.homeLabel}>Jump to:</span>
            <ul>
                <li>
                    <Link href={getEditorRoute()}>
                        <a className={styles.homeLabel}>Editor</a>
                    </Link>
                </li>
            </ul>
        </main>
    </div>
);

export default Home;