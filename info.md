Okay this is really helpful context. Looking at your schema and the search flow, here's how I'd think about it:

**The core insight** is that `items` in your app are really a global registry — they exist independently of lists, and lists just reference them via `list_items`. Progress also references items directly, independent of lists. So items aren't "owned" by lists, they're shared across the whole app.

With that, I'd structure features like this:

```
features/
├── lists/        — lists CRUD, list detail, list_items (adding/removing items from a list)
├── items/        — the global item registry, individual item detail page
├── search/       — the search UI + calling external APIs to import items
└── progress/     — progress logging and history for an item
```

Here's the reasoning for each split:

**`lists/`** owns everything about lists themselves and the `list_items` join — so the lists page, list detail page, the add/remove item from list flow, and sort order. It doesn't own what an item *is*, just which items belong to a list.

**`items/`** owns the item detail page and item-level display. When you click an item card, that's an items concern, not a lists concern.

**`search/`** is genuinely its own thing — it talks to external APIs, handles the import flow, and gets used from within the lists context but isn't owned by it. A dialog that lives in `features/search/` can be triggered from `features/lists/`.

**`progress/`** owns progress logging, history, and display. It gets rendered on the item detail page but it's its own slice of data and UI.

The cross-feature interaction you're worried about is totally normal and fine — `ListDetailPage` inside `features/lists/` can import `SearchDialog` from `features/search/` and `ProgressSummary` from `features/progress/`. Features import from each other, they just don't own each other's concerns.

The thing to avoid is features reaching into each other's internals — so `features/lists/` should only import what `features/search/` exposes through its `index.ts`, not dig into specific component files directly. That's exactly what the barrel exports are for.

# relating to the html file

Sure, let me lay this all out visually.A few things worth calling out:

`ItemCard.tsx` lives in `features/items/` and gets imported by `features/lists/` — that's a totally normal cross-feature import. The rule is just that `lists/` imports through `items/index.ts`, not directly into a specific file path.

`MediaTypeBadge` is the kind of thing that ends up in `components/` — it renders a pill like "movie" or "game" based on the `MediaType` enum, has no mutation logic, and will get used in item cards, search results, and probably the detail page. That's the profile of a shared presentational component.

`useListItems.ts` and `useItem.ts` are separate hooks because they serve different purposes — one fetches the items belonging to a specific list, the other fetches a single item by id for the detail page.

`progress/` is its own feature because it has its own data shape, its own set of mutations, and its own display logic that varies by `kind` (episode vs page vs percentage). That complexity justifies the separation even though it only ever appears on the item detail page.
