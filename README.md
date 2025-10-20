<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="https://raw.githubusercontent.com/LeoBorai/DeckMaster/refs/heads/main/docs/mtg.png" height="120" width="120" />
  </div>
  <h1 align="center">DeckMaster</h1>
  <h4 align="center">Magic: The Gathering Library Management Solution</h4>
</div>

## Environment Variables

| Variable      | Description                     | Required | Default Value  |
|---------------|---------------------------------|----------|----------------|
| `STORAGE_URL` | The Image Storage URL           | Yes      | N/A            |

## Deployment

### Server Side

#### Fly.io

1. Login to Fly.io

```bash
fly auth login
```

2. Run the deploy command

Make sure environment variables are set in your Fly.io app settings.

```bash
fly deploy -i ghcr.io/leoborai/deckmaster:latest
```
