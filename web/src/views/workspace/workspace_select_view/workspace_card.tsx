import { Button } from "base/button";
import styles from "./styles.module.scss";

export interface WorkspaceCardProps {
    workspaceId: string,
    name: string,
    description: string,
    onClick?(workspaceId: string): void,
    onInvite?(): void,
}

const WorkspaceCard = ({ workspaceId, name, description, onClick, onInvite }: WorkspaceCardProps) => {
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
            <Button variant="secondary" label="Delete" onClick={onInvite} />
        </button>
    );
};

export default WorkspaceCard;