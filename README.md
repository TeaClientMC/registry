# registry
A Public Registry of Mods/Texture-packs that are missing on CurseForge/Modrinth


# Channels

```mermaid
%%{init: {
    'theme': 'base',
    'themeVariables': {
        'gitInv0': '#ff0000',
        'gitInv1': '#ff0000',
        'git2': '#ff4444',
        'commitLabelFontSize': '15px'
    },
    'gitGraph': {
        'showCommitLabel':true,
        'mainBranchName': 'master',
        'rotateCommitLabel': true
    }
} }%%
gitGraph
    commit id:" "
    branch master
    branch unstable
    branch stable

    checkout unstable
    merge master id:"Automatic every 1 day"
    checkout stable 
    merge unstable id:"Automatic PR every 3-7 Days"
```
