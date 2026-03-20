/** A block within a page, returned from the backend. */
export interface Block {
  id: string;
  pageId: string;
  blockType: string;
  content: string;
  position: number;
  createdAt: string;
  updatedAt: string;
}

/** Editor state returned from the backend after each operation. */
export interface EditorState {
  pageId: string;
  blocks: Block[];
  isDirty: boolean;
}

/** Extended error kinds including block-related errors. */
export interface CommandError {
  kind:
    | "titleEmpty"
    | "titleTooLong"
    | "notFound"
    | "contentTooLong"
    | "invalidPosition"
    | "blockNotFound"
    | "cannotMoveUp"
    | "cannotMoveDown"
    | "storage";
  message: string;
}
