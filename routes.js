import React from 'react';
import ComponentCreator from '@docusaurus/ComponentCreator';

export default [
  {
    path: '/bot-box/blog',
    component: ComponentCreator('/bot-box/blog', '811'),
    exact: true
  },
  {
    path: '/bot-box/blog/archive',
    component: ComponentCreator('/bot-box/blog/archive', '54a'),
    exact: true
  },
  {
    path: '/bot-box/blog/authors',
    component: ComponentCreator('/bot-box/blog/authors', 'c7e'),
    exact: true
  },
  {
    path: '/bot-box/blog/authors/all-sebastien-lorber-articles',
    component: ComponentCreator('/bot-box/blog/authors/all-sebastien-lorber-articles', 'da3'),
    exact: true
  },
  {
    path: '/bot-box/blog/authors/yangshun',
    component: ComponentCreator('/bot-box/blog/authors/yangshun', 'f6c'),
    exact: true
  },
  {
    path: '/bot-box/blog/first-blog-post',
    component: ComponentCreator('/bot-box/blog/first-blog-post', 'd09'),
    exact: true
  },
  {
    path: '/bot-box/blog/long-blog-post',
    component: ComponentCreator('/bot-box/blog/long-blog-post', '416'),
    exact: true
  },
  {
    path: '/bot-box/blog/mdx-blog-post',
    component: ComponentCreator('/bot-box/blog/mdx-blog-post', 'd35'),
    exact: true
  },
  {
    path: '/bot-box/blog/tags',
    component: ComponentCreator('/bot-box/blog/tags', '24e'),
    exact: true
  },
  {
    path: '/bot-box/blog/tags/docusaurus',
    component: ComponentCreator('/bot-box/blog/tags/docusaurus', '734'),
    exact: true
  },
  {
    path: '/bot-box/blog/tags/facebook',
    component: ComponentCreator('/bot-box/blog/tags/facebook', '6b2'),
    exact: true
  },
  {
    path: '/bot-box/blog/tags/hello',
    component: ComponentCreator('/bot-box/blog/tags/hello', 'cf3'),
    exact: true
  },
  {
    path: '/bot-box/blog/tags/hola',
    component: ComponentCreator('/bot-box/blog/tags/hola', '91e'),
    exact: true
  },
  {
    path: '/bot-box/blog/welcome',
    component: ComponentCreator('/bot-box/blog/welcome', '3f9'),
    exact: true
  },
  {
    path: '/bot-box/markdown-page',
    component: ComponentCreator('/bot-box/markdown-page', 'cbb'),
    exact: true
  },
  {
    path: '/bot-box/docs',
    component: ComponentCreator('/bot-box/docs', '1cd'),
    routes: [
      {
        path: '/bot-box/docs',
        component: ComponentCreator('/bot-box/docs', '488'),
        routes: [
          {
            path: '/bot-box/docs',
            component: ComponentCreator('/bot-box/docs', '313'),
            routes: [
              {
                path: '/bot-box/docs/category/tutorial---basics',
                component: ComponentCreator('/bot-box/docs/category/tutorial---basics', '55d'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/bot-box/docs/category/tutorial---extras',
                component: ComponentCreator('/bot-box/docs/category/tutorial---extras', '4f1'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/bot-box/docs/intro',
                component: ComponentCreator('/bot-box/docs/intro', '59e'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/bot-box/docs/tutorial-basics/congratulations',
                component: ComponentCreator('/bot-box/docs/tutorial-basics/congratulations', '956'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/bot-box/docs/tutorial-basics/create-a-blog-post',
                component: ComponentCreator('/bot-box/docs/tutorial-basics/create-a-blog-post', '277'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/bot-box/docs/tutorial-basics/create-a-document',
                component: ComponentCreator('/bot-box/docs/tutorial-basics/create-a-document', 'a7f'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/bot-box/docs/tutorial-basics/create-a-page',
                component: ComponentCreator('/bot-box/docs/tutorial-basics/create-a-page', '626'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/bot-box/docs/tutorial-basics/deploy-your-site',
                component: ComponentCreator('/bot-box/docs/tutorial-basics/deploy-your-site', '1c4'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/bot-box/docs/tutorial-basics/markdown-features',
                component: ComponentCreator('/bot-box/docs/tutorial-basics/markdown-features', '5bf'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/bot-box/docs/tutorial-extras/manage-docs-versions',
                component: ComponentCreator('/bot-box/docs/tutorial-extras/manage-docs-versions', 'c86'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/bot-box/docs/tutorial-extras/translate-your-site',
                component: ComponentCreator('/bot-box/docs/tutorial-extras/translate-your-site', '1db'),
                exact: true,
                sidebar: "tutorialSidebar"
              }
            ]
          }
        ]
      }
    ]
  },
  {
    path: '/bot-box/',
    component: ComponentCreator('/bot-box/', '004'),
    exact: true
  },
  {
    path: '*',
    component: ComponentCreator('*'),
  },
];
