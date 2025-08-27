<code>
station acquire_tree_rs[train]
    if train[wreck] is set
        return train

    train[log] append [3, acquire_tree_rs:]

    if train[task] != install OR
       train[contained] != no OR
       tree_rs code is already downloaded
        return train

    # Set the version that we have

    # Define the latest version

    if our_version >= latest_version
        return train

    # Clone or download tree_rs code
    git clone tree_rs_repo

    if code fails to download
        train[wreck] = Run install again to download tree_rs code
        return train

    train[log] append [5, tree_rs source code aquired]

    return train
</code>