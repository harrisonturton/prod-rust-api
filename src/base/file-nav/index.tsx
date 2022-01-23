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
                    label: "notebooks",
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
                        {
                            kind: "file",
                            id: "2",
                            label: "Notes.docx",
                            active: false,
                        },
                    ]
                },
                {
                    kind: "folder",
                    id: "1-3",
                    label: "data",
                    collapsed: false,
                    active: false,
                    children: [
                        {
                            kind: "file",
                            id: "1-3-1",
                            label: "temperature.csv",
                            active: false,
                        },
                        {
                            kind: "file",
                            id: "1-3-2",
                            label: "tempature2.csv",
                            active: false,
                        },
                        {
                            kind: "file",
                            id: "1-3-3",
                            label: "readings.xlsx",
                            active: false,
                        },
                        {
                            kind: "file",
                            id: "1-3-4",
                            label: "mappings.geojson",
                            active: false,
                        },
                    ]
                },
            ],
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