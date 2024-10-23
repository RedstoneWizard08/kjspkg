# API

## Implementation status

- [x] `GET /api/v1/users/me` - Get information about the current user.
- [x] `GET /api/v1/auth/github/login` - GitHub sign-in flow
- [x] `GET /api/v1/auth/github/callback` - GitHub sign-in flow
- [x] `GET /api/v1/packages` - List packages
- [x] `GET /api/v1/packages/[id]` - Get information about a package
- [x] `GET /api/v1/packages/[id]/versions` - List package versions
- [x] `GET /api/v1/packages/[id]/versions/[id]` - Get information about a package version
- [x] `GET /api/v1/packages/[id]/versions/[id]/download` - Download a package
- [x] `PUT /api/v1/packages` - Publish a package
- [x] `PUT /api/v1/packages/[id]/versions` - Publish a package version
- [x] `PATCH /api/v1/packages/[id]` - Update information about a package
- [x] `PATCH /api/v1/packages/[id]/versions/[id]` - Update/overwrite a package version
- [x] `DELETE /api/v1/packages/[id]` - Delete an entire package
- [x] `DELETE /api/v1/packages/[id]/versions/[id]` - Delete a specific package version
