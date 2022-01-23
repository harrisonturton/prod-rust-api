import { default as FileTreeImpl } from "./file-tree";
import { FileTreeState, useFileTreeState } from "./file-tree-state";

const initialState: FileTreeState = {
    items: [
        {
            kind: "folder",
            id: "1",
            label: "My Project",
            collapsed: false,
            active: false,
            children: [
                {
                    kind: "file",
                    id: "1-1",
                    label: "README.md",
                    active: true,
                },
                {
                    kind: "folder",
                    id: "1-2",
                    label: "Notebooks",
                    collapsed: false,
                    active: false,
                    children: [
                        {
                            kind: "file",
                            id: "1-2-1",
                            label: "export.csv",
                            active: false,
                        },
                        {
                            kind: "file",
                            id: "1-2-2",
                            label: "notebook.ipnyb",
                            active: false,
                        },
                    ]
                },
            ],
        },
        {
            kind: "file",
            id: "2",
            label: "Notes",
            active: false,
        },
    ]
};

export default function() {
    const { state, collapseAll, toggleNode } = useFileTreeState(initialState);
    const onClick = (id: string) => toggleNode(id);
    const FileTree = () => (
        <FileTreeImpl
            items={state.items}
            toggleNode={onClick}
        />
    );

    return {
        state,
        collapseAll,
        FileTree
    }
};