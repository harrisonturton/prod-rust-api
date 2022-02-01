import { deepClone } from "base/util/state";
import { useState } from "react";

export interface FileTreeState {
    items: FileTreeItem[];
}

export type FileTreeItem = FileItem | DirectoryItem;

export interface FileItem {
    kind: "file";
    id: string;
    label: string;
    active: boolean;
}

export interface DirectoryItem {
    kind: "folder";
    id: string;
    label: string;
    collapsed: boolean;
    active: boolean;
    children: FileTreeItem[];
}

export const useFileTreeState = (initialState: FileTreeState) => {
    const [state, setState] = useState<FileTreeState>(initialState);

    const treeMap = (
        items: FileTreeItem[],
        func: (item: FileTreeItem) => FileTreeItem
    ) => {
        for (let i = 0; i < items.length; i++) {
            switch (items[i].kind) {
                case "file":
                    items[i] = func(items[i]);
                    continue;
                case "folder":
                    let newItem = func(items[i]);
                    if (newItem.kind == "file") {
                        items[i] = newItem;
                        continue;
                    }
                    treeMap(newItem.children, func);
                    items[i] = newItem;
                    break;
            }
        }
    };

    const toggleNode = (id: string) => {
        let newState = deepClone(state);
        treeMap(newState.items, (item) => {
            if (item.id == id && item.kind == "folder") {
                return { ...item, collapsed: !item.collapsed };
            }
            return item;
        });
        setState(newState);
    };

    const collapseAll = () => {
        let newState = deepClone(state);
        for (let item of newState.items) {
            switch (item.kind) {
                case "file":
                    continue;
                case "folder":
                    item.collapsed = true;
                    break;
            }
        }
        setState(newState);
    };

    return { state, collapseAll, toggleNode };
};
