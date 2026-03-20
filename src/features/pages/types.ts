/** A persisted page returned from the backend. */
export interface Page {
  id: string;
  title: string;
  createdAt: string;
  updatedAt: string;
}

/** Structured error returned from IPC commands. */
export interface CommandError {
  kind: "titleEmpty" | "titleTooLong" | "notFound" | "storage";
  message: string;
}

/** Arguments for the `create_page` IPC command. */
export interface CreatePageArgs {
  title: string;
}

/** Arguments for the `update_page_title` IPC command. */
export interface UpdatePageTitleArgs {
  id: string;
  title: string;
}

/** Arguments for the `delete_page` IPC command. */
export interface DeletePageArgs {
  id: string;
}

/** Arguments for the `get_page` IPC command. */
export interface GetPageArgs {
  id: string;
}
