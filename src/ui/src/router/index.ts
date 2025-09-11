import { createBrowserRouter } from "react-router-dom";

import { Home } from './Home';
import { Layout } from "../components/template/Layout";

export const router = createBrowserRouter([
  {
    id: 'root',
    path: "/",
    Component: Layout,
    children: [
      {
        path: "/",
        index: true,
        Component: Home,
      }
    ],
  },
]);
