import classNames from "classnames";
import { FileTreeItem } from "./file-tree-state";
import * as styles from "./styles.module.scss";

interface FileTreeProps {
    items: FileTreeItem[];
    toggleNode?: (id: string) => void;
}

const FileTree = ({ items, toggleNode }: FileTreeProps ) => (
    <div className={styles.root}>
        {items.map(item => {
            switch (item.kind) {
                case "file":
                    return (
                        <File
                            label={item.label}
                            indent={0}
                            active={item.active}
                        />
                    );
                case "folder":
                    return (
                        <Folder
                            id={item.id}
                            label={item.label}
                            collapsed={item.collapsed}
                            children={item.children}
                            indent={0}
                            active={item.active}
                            toggleNode={toggleNode}
                        />
                    );
            }
        })}
    </div>
);

interface FileProps {
    label: string;
    indent: number;
    active: boolean;
}

const minIndent = 10;
const indentSize = 7;

const File = ({ label, indent, active }: FileProps) => (
    <div
        className={classNames(styles.fileItem, { 
            [styles.indented]: indent > 0,
            [styles.active]: active,
        })}
        style={{ paddingLeft: `${minIndent + indent * indentSize}px` }}
    >
        <FileIcon />
        <span className={styles.fileItemLabel}>{label}</span>
    </div>
);

interface FolderProps {
    id: string;
    label: string;
    collapsed: boolean;
    children: FileTreeItem[];
    toggleNode?: (id: string) => void;
    indent: number;
    active: boolean;
}

const Folder = ({
    id,
    label,
    collapsed,
    children,
    toggleNode,
    indent,
    active,
}: FolderProps) => ( 
    <div className={styles.folderItem}>
        <div
            className={classNames(styles.folderItemLabelContainer, {
                [styles.indented]: indent > 0,
                [styles.active]: active,
            })}
            onMouseDown={(e: any) => toggleNode?.(id)}
            style={{
                paddingLeft: `${minIndent + indent * indentSize}px`,
            }}
        >
            <FolderIcon className={styles.folderIcon}/>
            <span className={styles.folderItemLabel}>{label}</span>
        </div>
        {!collapsed && children.map(item => {
            switch (item.kind) {
                case "file":
                    return (
                        <File
                            active={item.active}
                            label={item.label}
                            indent={indent + 1}
                        />
                    );
                case "folder":
                    return (
                        <Folder
                            id={item.id}
                            label={item.label}
                            collapsed={item.collapsed}
                            children={item.children}
                            indent={indent + 1}
                            active={item.active}
                            toggleNode={toggleNode}
                        />
                    );
            }
        })}
    </div>
);

const FileIcon = () => (
    <div className={styles.fileIcon}>
    </div>
);

const FolderIcon = ({ className }: { className?: string }) => (
    <svg className={className} width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M0 5C0 4.44772 0.447715 4 1 4H11.7444C12.0821 4 12.397 4.17045 12.5817 4.45321L14.5714 7.5H23C23.5523 7.5 24 7.94772 24 8.5V19C24 19.5523 23.5523 20 23 20H1C0.447716 20 0 19.5523 0 19V5Z" fill="inherit"/>
    </svg>
);

export default FileTree;