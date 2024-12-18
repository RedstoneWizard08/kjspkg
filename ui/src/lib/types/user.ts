export interface User {
    /**
     * The ID of this user.
     */
    id: number;

    /**
     * The username of this user.
     */
    username: string;

    /**
     * This user's GitHub numerical ID.
     */
    github_id: number;

    /**
     * Whether the user is an admin or not.
     */
    admin: boolean;
}
