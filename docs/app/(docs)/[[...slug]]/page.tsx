import type { Metadata } from 'next';
import { notFound } from 'next/navigation';
import { DocsBody, DocsPage } from 'fumadocs-ui/layouts/docs/page';
import { getMDXComponents } from '@/components/mdx';
import { source } from '@/lib/source';

type PageProps = {
  params: Promise<{
    slug?: string[];
  }>;
};

export function generateStaticParams() {
  return source.generateParams();
}

export async function generateMetadata({
  params,
}: PageProps): Promise<Metadata> {
  const { slug } = await params;
  const page = source.getPage(slug ?? []);

  if (!page) {
    return {};
  }

  return {
    description: page.data.description,
    openGraph: {
      description: page.data.description,
      title: page.data.title,
      type: 'article',
      url: page.url,
    },
    title: page.data.title,
  };
}

export default async function Page({ params }: PageProps) {
  const { slug } = await params;
  const page = source.getPage(slug ?? []);

  if (!page) {
    notFound();
  }

  const MDX = page.data.body;

  return (
    <DocsPage tableOfContent={{ style: 'clerk' }} toc={page.data.toc}>
      <DocsBody>
        <MDX components={getMDXComponents()} />
      </DocsBody>
    </DocsPage>
  );
}
