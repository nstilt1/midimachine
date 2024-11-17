import Image from 'next/image';
import Link from 'next/link';
import '../styles/MainSiteStyles.css';

const Navbar = ( { whenClickedLogo } ) => {
  return (
    <header id="_header-127-19" className="oxy-header-wrapper oxy-sticky-header oxy-overlay-header oxy-header">
      <div id="_header_row-128-29" className="oxy-header-row">
        <div className="oxy-header-container">
            <div id="_header_left-129-29" className='oxy-header-left'>
                <a id="link-130-29" className='ct-link saas-logo' target="_self">
                    <Image id="image-131-29" className='ct-image abc-logo' src="/abc_logo_5_c2.png" alt="Logo" height={80} width={100} onClick={whenClickedLogo} />
                </a>
            </div>
            <div id="_header_center-132-29" className='oxy-header-center'>

            </div>
            <div id="_header_right-133-29" className='oxy-header-right'>
                <div id="div_block-134-29" className="ct-div-block">
                <nav id="_nav_menu-135-29" className="oxy-nav-menu oxy-nav-menu-dropdowns oxy-nav-menu-dropdown-arrow">
                    <div className="oxy-menu-toggle">
                        <div className="oxy-nav-menu-hamburger-wrap">
                            <div className="oxy-nav-menu-hamburger">
                                <div className="oxy-nav-menu-hamburger-line"></div>
                                <div className="oxy-nav-menu-hamburger-line"></div>
                                <div className="oxy-nav-menu-hamburger-line"></div>
                            </div>
                        </div>
                    </div>
                    <div className="menu-main-menu-signed-in-container">
                        <ul id="menu-main-menu-signed-in-1" className="oxy-nav-menu-list">
                            <li className="menu-item menu-item-type-post_type menu-item-object-page menu-item-2080">
                                <a href="https://alteredbrainchemistry.com/shop/">Shop</a>
                            </li>
                            <li className="menu-item menu-item-type-post_type menu-item-object-page menu-item-2078"><a href="https://alteredbrainchemistry.com/my-cart/">My Cart</a></li>
                            <li className="menu-item menu-item-type-post_type menu-item-object-page menu-item-2174"><a href="https://alteredbrainchemistry.com/blog/">Blog</a></li>
                </ul>
            </div>
        </nav>
        <a id="link_text-136-29" className="ct-link-text" href="https://buymeacoffee.com/alteredbrainchemistry" target="_blank">SUPPORT US</a>
    </div>
    </div>
        </div>
      </div>
    </header>
  );
};

export default Navbar