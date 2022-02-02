import PageContainer from "base/page_container";
import styles from "./styles.module.scss";
import React from "react";
import { getEditorRoute, Router } from "base/router";
import WorkspaceCard from "./workspace_card";

const WorkspaceSelectView = () => {
    let router = new Router();

    const onCardClick = (workspaceId: string) => {
        let editorRoute = getEditorRoute();
        router.pushRoute(editorRoute)
    };

    return (
        <PageContainer>
            <div className={styles.workspaceListContainer}>
                <h1>My Workspaces</h1>
                <WorkspaceCard
                    workspaceId="workspaceId"
                    name="Editor"
                    description="Online code editor"
                    onClick={onCardClick}
                />
                <WorkspaceCard
                    workspaceId="workspaceId"
                    name="Final Assignment (COMP3310)"
                    description="ASM Synthesizer"
                    onClick={onCardClick}
                />
            </div>
        </PageContainer>
    );
};

export default WorkspaceSelectView;