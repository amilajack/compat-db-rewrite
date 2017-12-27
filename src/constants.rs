pub struct BrowserNameToCaniuseMappings {
    chrome: String,
    firefox: String,
    opera: String,
    safari: String,
    MicrosoftEdge: String,
    internet_explorer: String,
}

pub struct CaniuseToSeleniumMappings {
    chrome: String,
    firefox: String,
    opera: String,
    safari: String,
    ie: String,
    edge: String,
}

pub struct FixedBrowserVersions {
    browser_name: String,
    platform: String,
    version: String,
}

pub fn mappings() -> (BrowserNameToCaniuseMappings, [FixedBrowserVersions; 15]) {
    let browser_name_to_caniuse_mappings = BrowserNameToCaniuseMappings {
        chrome: String::from("chrome"),
        firefox: String::from("firefox"),
        opera: String::from("opera"),
        safari: String::from("safari"),
        MicrosoftEdge: String::from("edge"),
        internet_explorer: String::from("ie"),
    };

    let fixed_browser_versions = [FixedBrowserVersions {
                                      browser_name: String::from("opera"),
                                      platform: String::from("Windows 7"),
                                      version: String::from("12.12"),
                                  },
                                  FixedBrowserVersions {
                                      browser_name: String::from("opera"),
                                      platform: String::from("Windows 7"),
                                      version: String::from("11.64"),
                                  },
                                  // Edge: String::from(2 version)s
                                  FixedBrowserVersions {
                                      browser_name: String::from("MicrosoftEdge"),
                                      platform: String::from("Windows 10"),
                                      version: String::from("14.14393"),
                                  },
                                  FixedBrowserVersions {
                                      browser_name: String::from("MicrosoftEdge"),
                                      platform: String::from("Windows 10"),
                                      version: String::from("13.10586"),
                                  },
                                  // Safari: String::from(5 version)s
                                  FixedBrowserVersions {
                                      browser_name: String::from("safari"),
                                      platform: String::from("OS X 10.12"),
                                      version: String::from("10.0"),
                                  },
                                  FixedBrowserVersions {
                                      browser_name: String::from("safari"),
                                      platform: String::from("OS X 10.11"),
                                      version: String::from("9.0"),
                                  },
                                  FixedBrowserVersions {
                                      browser_name: String::from("safari"),
                                      platform: String::from("OS X 10.10"),
                                      version: String::from("8.0"),
                                  },
                                  FixedBrowserVersions {
                                      browser_name: String::from("safari"),
                                      platform: String::from("OS X 10.9"),
                                      version: String::from("7.0"),
                                  },
                                  FixedBrowserVersions {
                                      browser_name: String::from("safari"),
                                      platform: String::from("OS X 10.8"),
                                      version: String::from("6.0"),
                                  },
                                  // IE: String::from(6 version)s
                                  FixedBrowserVersions {
                                      browser_name: String::from("internet explorer"),
                                      platform: String::from("Windows 7"),
                                      version: String::from("11.0"),
                                  },
                                  FixedBrowserVersions {
                                      browser_name: String::from("internet explorer"),
                                      platform: String::from("Windows 7"),
                                      version: String::from("10.0"),
                                  },
                                  FixedBrowserVersions {
                                      browser_name: String::from("internet explorer"),
                                      platform: String::from("Windows 7"),
                                      version: String::from("9.0"),
                                  },
                                  FixedBrowserVersions {
                                      browser_name: String::from("internet explorer"),
                                      platform: String::from("Windows 7"),
                                      version: String::from("8.0"),
                                  },
                                  FixedBrowserVersions {
                                      browser_name: String::from("internet explorer"),
                                      platform: String::from("Windows XP"),
                                      version: String::from("7.0"),
                                  },
                                  FixedBrowserVersions {
                                      browser_name: String::from("internet explorer"),
                                      platform: String::from("Windows XP"),
                                      version: String::from("6.0"),
                                  }];

    return (browser_name_to_caniuse_mappings, fixed_browser_versions);
}
