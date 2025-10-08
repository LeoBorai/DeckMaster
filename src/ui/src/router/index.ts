import { createBrowserRouter } from "react-router-dom";

import { Layout } from "../components/template/Layout";
import { Card } from "./Card";
import { Home } from "./Home";

export const router = createBrowserRouter([
  {
    id: "root",
    path: "/",
    Component: Layout,
    children: [
      {
        path: "/",
        index: true,
        Component: Home,
      },
      {
        path: "/cards/:id",
        Component: Card,
      },
    ],
  },
]);
