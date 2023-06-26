# Rusty Tusks

This is the __Walrus__ operator built in `rustlang`

## Goals

## How it works

### Project setup

project setup is not fully baked out yet

```
rusty-tusks
├── Cargo.lock
├── Cargo.toml
├── Makefile
├── output
├── README.md
├── resources
│   └── collins.yaml
└── src
    ├── controller.rs
    ├── lib.rs
    ├── main.rs
    └─── models
        ├── mod.rs
        ├── pod.rs
        └── state.rs
```

### ArgoCD setup

In order to test out ArgoCD I needed to get it setup in my cluster. Here are the steps that I went through.

Install the manifests and create namespace
```bash
kubectl create namespace argocd
kubectl apply -n argocd -f https://raw.githubusercontent.com/argoproj/argo-cd/stable/manifests/install.yaml
```

In order to auth with the cli / `kubectl` you need to have the namespace set in the current context
```bash
$ k config set-context --current --namespace argocd
Context "my-context" modified.

$ argocd login my-context --core
Context 'kubernetes' updated

$ argocd app list
NAME                    CLUSTER                         NAMESPACE  PROJECT  STATUS  HEALTH   SYNCPOLICY  CONDITIONS  REPO                                      PATH       TARGET
...
```

Forward the UI via the proxy server
```bash
kubectl port-forward svc/argocd-server -n argocd 8080:443
```

Login to the `admin` account and get the password
```bash
argocd admin initial-password -n argocd
```

Then you need to setup the repo & application. For now I just had it deploy the sample resources I have in the [`/resources`](/resources) directory

I need to figure out how to get the images pushed to my image server and then have it
use the creds to be able to pull from my server...

OKAY so the way I wanna do this is create another SA on my docker registry to have the cluster Argo is running in be able to get the images

Actually this is unnecessary. Instead I need to create a new service account and apply it to the registry. The steps are above. Either way I still am interested to check out more about the project below:

Going to look into this project: [argocd-image-updater](https://argocd-image-updater.readthedocs.io/en/stable/install/installation/)

> Dont forget to make sure the namespace is created for the operator to run in!
>
> ```bash
> k create ns rusty-tusks
> ```


#### Extra info and context


Walruses are large marine mammals that belong to the pinniped family, which also includes seals and sea lions. They are primarily found in the Arctic regions of the Northern Hemisphere. Here are some key points about where walruses typically live:

###### Geographic Range

Walruses have a circumpolar distribution, inhabiting coastal areas of the Arctic Ocean and its adjoining seas. Their range spans from the eastern coast of Greenland across the northern coasts of North America and Eurasia, extending as far west as the Bering Strait.

###### Preferred Habitat

Walruses rely on sea ice as a platform for various activities such as resting, giving birth, and finding food. During the summer, when the sea ice retreats, they typically migrate to shallow continental shelves and coastal areas. These regions provide opportunities for bottom-feeding on benthic organisms like clams, snails, and other invertebrates.

###### Specific Locations
Some notable locations where walruses are known to inhabit include the Bering Sea, the Chukchi Sea, the Laptev Sea, the Kara Sea, and the Hudson Bay.

###### Species
>   __Atlantic Walrus__ (Odobenus rosmarus rosmarus): This species is found in the Atlantic Ocean, particularly in the coastal regions of northeastern Canada and western Greenland.

>   __Pacific Walrus__ (Odobenus rosmarus divergens): The Pacific walrus is located in the Pacific Ocean, primarily in the seas around Alaska, Russia, and the northern parts of Canada.

>   __Laptev Walrus__ (Odobenus rosmarus laptevi): This subspecies or population of walruses is restricted to the Laptev Sea in the Arctic Ocean, near the coast of Siberia.
