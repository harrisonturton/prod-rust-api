import PageContainer from "base/page_container";
import { Button } from "base/button";
import styles from "./styles.module.scss";
import React from "react";
import { getEditorRoute, Router } from "base/router";

const WorkspaceSelectView = () => {
    let router = new Router();

    const onCardClick = (workspaceId: string) => {
        let editorRoute = getEditorRoute();
        router.pushRoute(editorRoute)
    };

    return (
        <PageContainer>
            <div className={styles.workspaceListContainer}>
                <WorkspaceItem
                    workspaceId="workspaceId"
                    name="Editor"
                    description="Online code editor"
                    onClick={onCardClick}
                />
            </div>
        </PageContainer>
    );
};

export interface WorkspaceCardProps {
    workspaceId: string,
    name: string,
    description: string,
    onClick?(workspaceId: string): void,
    onInvite?(): void,
}

const WorkspaceItem = ({ workspaceId, name, description, onClick, onInvite }: WorkspaceCardProps) => {
    const handleCardClick = (e: React.MouseEvent<HTMLButtonElement>) => {
        onClick?.(workspaceId);
        e.preventDefault();
    };
    return (
        <button className={styles.card} onClick={handleCardClick}>
            <div className={styles.cardLeft}>
                <div className={styles.graphic}></div>
                <div className={styles.detailsContainer}>
                    <span className={styles.name}>{name}</span>
                    <span className={styles.description}>{description}</span>
                </div>
            </div>
            <Button variant="secondary" label="Invite" onClick={onInvite} />
        </button>
    );
};

export default WorkspaceSelectView;