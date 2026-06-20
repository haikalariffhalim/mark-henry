# messiRS

## Convert from Savage (SVG) to an Icon (ico, png,...) just like The Goat himself

**Tables (1):**

| Table            |      API Usage                                       |
| ---------------- | -----------------------------------------------------|
| 						     |  	api.useMessi.savage.to.penang({									  |
|									 |			ctx:svg{ png,jpg, ...rest}   					 			    |
|									 |		});																							  |
|									 |		handler: await (ctx,args)													|
| `svg to png` 		 | 								↓↓↓↓↓						        ↓↓↓↓↓					|
										  filePath:{"/../..svg",(messi.RS."to/../somewhere")} |
|								   |																											|
|	`png to svg`		 |																											|
| `svg to pdf`   	 |                                                      |
| `svg to html`    |        																							|
| `html to pdf`    |       																								|
| `pdf to csv`     |  																						        |

**Current API:**

```ts

// Mounting
http.route({
  path: "/api/messi",
  method: "POST",
  handler: messi.httpHandler(),
});


```steps
User catches ghost
       │
       ▼
CollectionContext.catchGhost()
	
  ├── 1. setCollection(...)     ← instant optimistic UI update
  │
  ├── 2. AsyncStorage.setItem() ← offline-safe local backup
  │
  └── 3. convexHttp.mutation(convexFns.catchGhost, { deviceId, ghostId, ... })
                │
                ▼
         Convex Cloud runs convex/collection.ts → catchGhost()
                │
                └── upserts row in ghostCollection table
```

### Using messiRS Custom Validator 



```rs

api.useMessi.savage.to.penang (ctx:(........),{args.})
	
[!]
		api.useMessi.savage.to.penang ({
			ctx:svg{png,jpg, ...rest}    		 ←── [1] convert svg to png/jpg			
		handler: await (ctx,args){
			
		filePath:{"/../..svg",  	  ←── [2] input file path to convert 
			
			(messi.RS."to/somefile/somewhere")} 
		)}
		
		//... return from your function with intended value
		
		
```

```sh

[1]
├── [2] savage.to.penang     ← convert svg to png
│
│
├── [3] savage.to.jepang     ← convert svg to jpg
│
│
└── [4] savage.to.econsave   ← convert svg to ico	

```


### State Transitions

```sh
┌─────────────────┐
│  Start parsing  │
└────────┬────────┘
         │
         ▼
┌─────────────────────────────────────────┐
│  Iterate through events                 │
└────────┬────────────────────────────────┘
         │
    ┌────┴───── ┐
    │           │
    ▼           ▼
 HEADING    NON-HEADING
    │           │
    │      ┌────┴─────────────┐
    │      │  Buffer in       │
    │      │  pending vector  │
    │      └──────────────────┘
    │
    ├─────────────────────────┐
    │ Flush pending buffer    │
    │ (if not empty)          │
    └────────────┬────────────┘
                 │
    ┌────────────┴────────────┐
    │ Extract heading text    │
    └────────────┬────────────┘
                 │
    ┌────────────┴────────────┐
    │ Generate ID via slugify │
    └────────────┬────────────┘
                 │
         ┌───────┴────────┐
         │                │
    ┌────▼───┐  ┌────────▼────────┐
    │   H2   │  │  H3 or other    │
    └────┬───┘  └────────┬────────┘
         │               │
    ┌────▼───────────┐   │
    │ Close H2?      │   │
    │ Open new <sec> │   │
    └────┬───────────┘   │
         │               │
    ┌────▼───────────────▼──── ┐
    │ Generate HTML for heading│
    └────┬─────────────────────┘
         │
    ┌────▼──────────────────────┐
    │ Add entry to TOC          │
    └────┬──────────────────────┘
         │
    ┌────▼──────────────────────┐
    │ Move to next event        │
    └────┬──────────────────────┘
         │
    ┌────▴──────────────────────┐
    │ More events?              │
    └────┬──────────┬───────────┘
         │ YES      │ NO
         │          └─────┐
         └────────┬───────┘
                  │
         ┌────────▼──────────────┐
         │ Close open H2 section │
         │ Add closing </ul>     │
         │ Return RenderedPage   │
         └────────────────────── ┘
```

// Queries
await messi.savagetopenang(ctx, { appUserId, entitlementId: "premium" });
await messi.getActiveEntitlements(ctx, { appUserId });
await messi.getAllEntitlements(ctx, { appUserId });
await messi.getActiveSubscriptions(ctx, { appUserId });
await messi.getAllSubscriptions(ctx, { appUserId });
await messi.getCustomer(ctx, { appUserId });
await messi.getExperiment(ctx, { appUserId, experimentId });
await messi.getExperiments(ctx, { appUserId });
```

**Output Result: **
```
icons/
├── icon.ico
├── icon.icns
├── icon-512.png
├── icon_16x16.png
├── icon_32x32.png
├── icon_48x48.png
├── icon_64x64.png
├── icon_128x128.png
├── icon_256x256.png
├── icon_512x512.png
├── icon_1024x1024.png
├── apple-touch-icon.png
├── android-chrome-192.png
├── android-chrome-512.png
└── og-image.png
```
