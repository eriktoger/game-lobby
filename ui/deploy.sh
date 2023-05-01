#!/bin/bash
HASH=$(git rev-parse HEAD;)
npm run build && netlify deploy --prod --dir ../static --message $HASH