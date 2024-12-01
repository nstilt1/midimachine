import Image from 'next/image';
import Link from 'next/link';

const Navbar = ( { whenClickedLogo } ) => {
  return (
    <nav className="bg-gray-800 p-4">
      <div className="container mx-auto flex justify-between items-center">
        <div className="flex items-center">
        <Image src="/abc_logo_5_c2.png" alt="Logo" width={80} height={80} onClick={whenClickedLogo} />
          <Link href="https://alteredbrainchemistry.com" >
            Home
          </Link>
        </div>
        <div className="flex space-x-4">
          <Link href="https://alteredbrainchemistry.com/shop" className="text-white">
            Shop
          </Link>
          <Link href="https://alteredbrainchemistry.com/blog" className="text-white">
            Blog
          </Link>
        </div>
      </div>
    </nav>
  );
};

export default Navbar