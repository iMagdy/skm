import { loader } from 'fumadocs-core/source';
import { docs } from 'collections/server';

export const source = loader({
  baseUrl: '/',
  source: docs.toFumadocsSource(),
  slugs(file) {
    if (file.path === 'README.md') {
      return [];
    }

    if (file.path === 'RELEASE_NOTES.md') {
      return ['release-notes'];
    }

    return undefined;
  },
});
